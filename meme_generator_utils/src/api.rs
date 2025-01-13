use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{mpsc as std_mpsc, LazyLock, Mutex},
    thread,
};

use serde_json::{json, Value};
use tokio::sync::mpsc;
use tracing::warn;

use crate::config::CONFIG;

type AsyncResult = Pin<Box<dyn Future<Output = Option<Value>> + Send + Sync>>;
type AsyncFunction = Box<dyn Fn(Value) -> AsyncResult + Send + Sync>;

static APIS: LazyLock<HashMap<String, AsyncFunction>> = LazyLock::new(|| {
    let mut apis = HashMap::new();
    if let Some(api) = register_translate() {
        apis.insert("translate".to_string(), api);
    }
    apis
});

struct ApiManager {
    task_sender: mpsc::Sender<(String, Value)>,
    result_receiver: std_mpsc::Receiver<Option<Value>>,
}

impl ApiManager {
    fn new() -> Self {
        let (task_sender, mut task_receiver) = mpsc::channel::<(String, Value)>(32);
        let (result_sender, result_receiver) = std_mpsc::channel::<Option<Value>>();

        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                while let Some((api_name, params)) = task_receiver.recv().await {
                    if let Some(api) = APIS.get(&api_name) {
                        let result = api(params).await;
                        result_sender.send(result).unwrap();
                    } else {
                        result_sender.send(None).unwrap();
                    }
                }
            });
        });

        Self {
            task_sender,
            result_receiver,
        }
    }

    fn send_task(&self, name: String, params: Value) {
        self.task_sender.blocking_send((name, params)).unwrap();
    }

    fn recv_result(&self) -> Option<Value> {
        self.result_receiver.recv().unwrap()
    }
}

static API_MANAGER: LazyLock<Mutex<ApiManager>> = LazyLock::new(|| Mutex::new(ApiManager::new()));

fn call_api(name: &str, params: Value) -> Option<Value> {
    let api_manager = API_MANAGER.lock().unwrap();
    api_manager.send_task(name.to_string(), params);
    api_manager.recv_result()
}

async fn translate_async(params: Value) -> Option<Value> {
    let text = params["text"].as_str()?;
    let lang_from = params["lang_from"].as_str().unwrap_or("auto");
    let lang_to = params["lang_to"].as_str().unwrap_or("zh");

    let appid = CONFIG.api.baidu_trans_appid.clone().unwrap();
    let apikey = CONFIG.api.baidu_trans_apikey.clone().unwrap();

    let salt = chrono::Utc::now().timestamp_millis().to_string();
    let sign_raw = format!("{}{}{}{}", appid, text, salt, apikey);
    let sign = format!("{:x}", md5::compute(sign_raw.as_bytes()));
    let params = json!({
        "q": text,
        "from": lang_from,
        "to": lang_to,
        "appid": appid,
        "salt": salt,
        "sign": sign,
    });
    let url = "https://fanyi-api.baidu.com/api/trans/vip/translate";
    let client = reqwest::Client::new();
    let resp = match client.get(url).query(&params).send().await {
        Ok(resp) => resp,
        Err(err) => {
            warn!("请求百度翻译 API 失败: {:?}", err);
            return None;
        }
    };
    let result: Value = match resp.json().await {
        Ok(result) => result,
        Err(err) => {
            warn!("解析百度翻译 API 响应失败: {:?}", err);
            return None;
        }
    };
    let trans_result: &str = match result["trans_result"][0]["dst"].as_str() {
        Some(result) => result,
        None => {
            warn!("百度翻译 API 响应格式错误: {:?}", result);
            return None;
        }
    };
    return Some(json!({
        "result": trans_result
    }));
}

fn translate_wrapper(params: Value) -> AsyncResult {
    Box::pin(async move { translate_async(params).await })
}

fn register_translate() -> Option<AsyncFunction> {
    let appid = &CONFIG.api.baidu_trans_appid;
    let apikey = &CONFIG.api.baidu_trans_apikey;
    if appid.is_none() || apikey.is_none() {
        warn!("\"baidu_trans_appid\" 或 \"baidu_trans_apikey\" 未设置，请检查配置文件！");
        return None;
    }
    Some(Box::new(translate_wrapper))
}

pub fn translate(text: &str, lang_to: &str) -> Option<String> {
    let params = json!({
        "text": text,
        "lang_to": lang_to,
    });
    let result = call_api("translate", params)?;
    Some(result["result"].as_str().unwrap_or("").to_string())
}
