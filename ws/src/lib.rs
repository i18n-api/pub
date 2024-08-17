use std::{borrow::Cow, ops::ControlFlow};

use anyhow::Result;
use axum::extract::ws::close_code::{AWAY, NORMAL};
use client::Client;
use dashmap::DashMap;
use futures::{future::join_all, stream::SplitSink};
use futures_util::{SinkExt, StreamExt};
use gxhash::HashMap;
use r::{fred::interfaces::KeysInterface, R};
use radix_str::radix_str;
use t3::{
  axum::extract::{
    ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    Path,
  },
  IntoResponse,
};

pub type Sender = SplitSink<WebSocket, Message>;

#[static_init::dynamic]
pub static UID_CLIENT_ID_WS: DashMap<u64, HashMap<u64, Sender>> = DashMap::new();

pub static MSG_USER: &str = concat!(radix_str!(0, 36), "[");

pub async fn to_other(client_id: u64, uid: u64, msg: Message) {
  if let Some(mut li) = UID_CLIENT_ID_WS.get_mut(&uid) {
    let mut vec = Vec::with_capacity(li.len() - 1);
    for (id, ws) in li.iter_mut() {
      if *id != client_id {
        vec.push(ws.send(msg.clone()));
      }
    }
    join_all(vec).await;
  }
}

pub async fn to_all(uid: u64, msg: Message) {
  if let Some(mut li) = UID_CLIENT_ID_WS.get_mut(&uid) {
    let mut vec = Vec::with_capacity(li.len());
    for (_, ws) in li.iter_mut() {
      vec.push(ws.send(msg.clone()));
    }
    join_all(vec).await;
  }
}

pub async fn get(
  ws: WebSocketUpgrade,
  client: Client,
  Path((uid, ver)): Path<(String, String)>,
) -> re::Result<impl IntoResponse> {
  let uid = u64::from_str_radix(&uid, 36).unwrap_or(0);
  client.uid_logined(uid).await?;
  let ver = u64::from_str_radix(&ver, 36).unwrap_or(0);
  Ok(ws.on_upgrade(move |socket| open(socket, client.id, uid, ver)))
}

pub async fn msg_user_by_uid_bin(uid_bin: impl AsRef<[u8]>) -> Result<Message> {
  let json = user::by_id_bin(uid_bin.as_ref()).await?.to_json();
  Ok(Message::Text(MSG_USER.to_owned() + &json))
}

async fn open(socket: WebSocket, client_id: u64, uid: u64, ver: u64) {
  let (sender, mut receiver) = socket.split();

  tokio::spawn(async move {
    {
      UID_CLIENT_ID_WS
        .entry(uid)
        .or_default()
        .insert(client_id, sender);
    }
    let uid_bin = &intbin::u64_bin(uid)[..];
    let now_ver: u64 = R.incr_by(xbin::concat!(user::K::VER, uid_bin), 0).await?;
    if now_ver != ver {
      let user = msg_user_by_uid_bin(uid_bin).await?;

      if let Some(mut client_id_sender) = UID_CLIENT_ID_WS.get_mut(&uid) {
        if let Some(sender) = client_id_sender.get_mut(&client_id) {
          if let Err(e) = sender.send(user).await {
            tracing::error!("{uid} {client_id} {e}");
            let _ = sender
              .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from(""),
              })))
              .await;
          }
        }
      }
    }
    Ok::<_, anyhow::Error>(())
  });

  // let mut send_task = tokio::spawn(async move {
  //   let n_msg = 20;
  //   for i in 0..n_msg {
  //     // 任何错误直接退出
  //     if sender
  //       .send(Message::Text(format!("服务器消息{i}...")))
  //       .await
  //       .is_err()
  //     {
  //       return i;
  //     }
  //
  //     tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
  //   }
  //
  //   println!("正在发送关闭...");
  //   if let Err(e) = sender
  //     .send(Message::Close(Some(CloseFrame {
  //       code: axum::extract::ws::close_code::NORMAL,
  //       reason: Cow::from("再见"),
  //     })))
  //     .await
  //   {
  //     println!("无法发送关闭:{e},也许没关系?");
  //   }
  //   n_msg
  // });

  loop {
    if let Some(msg) = receiver.next().await {
      match msg {
        Ok(msg) => {
          if process_message(msg).is_break() {
            break;
          }
        }
        Err(msg) => {
          tracing::error!("{msg}");
          break;
        }
      };
    }
  }

  let removed = {
    UID_CLIENT_ID_WS
      .remove_if(&uid, |_, m| m.len() == 1)
      .is_some()
  };
  if !removed {
    if let Some(mut m) = UID_CLIENT_ID_WS.get_mut(&uid) {
      m.remove(&client_id);
    }
  }

  // tokio::select! {
  //     rv_a = (&mut send_task) => {
  //         match rv_a {
  //             Ok(a) => println!("向客户端发送了{}条消息",a),
  //             Err(a) => println!("发送消息时出错:{:?}",a)
  //         }
  //         recv_task.abort();
  //     },
  //     rv_b = (&mut recv_task) => {
  //         match rv_b {
  //             Ok(b) => println!("接收到{}条消息",b),
  //             Err(b) => println!("接收消息时出错:{:?}",b)
  //         }
  //         send_task.abort();
  //     }
  // }
}

pub fn process_message(msg: Message) -> ControlFlow<(), ()> {
  if let Message::Close(c) = msg {
    if let Some(cf) = c {
      if ![NORMAL, AWAY].contains(&cf.code) {
        tracing::info!("websocket close {} {}", cf.code, cf.reason);
      }
    }
    return ControlFlow::Break(());
  }
  ControlFlow::Continue(())
}
