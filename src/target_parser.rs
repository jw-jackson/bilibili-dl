use reqwest::Client;
use tokio::sync::mpsc;
use url::{Url};
use serde::Serialize;
use reqwest::Response;
use serde::Deserialize;


use tokio::task::JoinSet;
use tokio::task; 
use crate::Target::VIDEO;
use crate::Target::LIVE;




#[derive(Debug)]
pub struct Video{
    pub aid: Option<String>,
    pub bvid: Option<String>,
    pub cid: Option<String>,
    pub flags: Option<u8>,
    pub title: Option<String>,
    pub page_range: (usize, usize),
    pub page_id :Option<u8>,
}




#[derive(Debug)]
pub enum Target {
    /* 普通视频 */
    VIDEO(Video),
    /* 直播 */
    LIVE,
}




pub struct TargetParser{
    client: &'static Client,
    receiver: mpsc::Receiver<Target>,
    sender: mpsc::Sender<Video>,
}


impl TargetParser {
    pub fn new(client: &'static Client, receiver: mpsc::Receiver<Target>, sender: mpsc::Sender<Video>) -> TargetParser{
        TargetParser {
            client,
            receiver,
            sender
        }
    }
    pub async fn start(mut self) {
        let mut set = JoinSet::new();
        while let Some(target) = self.receiver.recv().await {
            set.spawn(target_parse(self.client, target, self.sender.clone()));
            //target_parse(self.client, target).await;

        }
        while let Some(_) = set.join_next().await {}
    }
}




async fn target_parse(client: &Client, target: Target, sender: mpsc::Sender<Video>) {
    match target {
        VIDEO(video) => {
            proc_video(client, video, sender).await;
        },
        LIVE => {
            proc_live();
        }
    }
    //println!("{:?}", target);
}




async fn proc_video(client: &Client, mut video:Video, sender: mpsc::Sender<Video>) {
    let mut url = Url::parse("https://api.bilibili.com/x/web-interface/view").expect("Failed to parse URL");

    if let Some(ref bvid) = video.bvid {
        url.query_pairs_mut()
            .append_pair("bvid", bvid);
    }
    let response = client.get(&url.to_string()).send().await.unwrap();
    //println!("{:?}", response);

    #[derive(Deserialize, Serialize, Debug)]
    struct Response <T>{
        code: i32,
        message: String,
        ttl: i32,
        data: T,
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Page {
        cid: i32,
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        bvid: String,
        videos: i32,
        title: String,
        cid: i32,
        pages: Vec<Page>,
    }
    let response:Response<Data>= response.json().await.unwrap();
    //println!("{:#?}", response.data);
    /*
    let mut page_start: u8;
    let mut page_end: u8;
    if let Some((page_start, page_end)) = video.page_range {}
    else {
        
    }
    */

    //video.title = Some(response.data.title);
    let (page_start, page_end) = video.page_range;
    if page_start == page_end {
        video.title = Some(response.data.title);
        video.cid = Some(response.data.pages[page_start - 1].cid.to_string());
        sender.send(video).await.unwrap();
    }
    else {
        for i in page_start..=page_end {
            todo!();
        }
    }
}


fn proc_live(){
    todo!();
}





// https://www.bilibili.com/video/BV1X94y137HR/?spm_id_from=333.1007.tianma.2-1-4.click
// https://www.bilibili.com/video/BV1Eb411u7Fw?p=5

/* start url parser */
/* way: bid vid ? ....*/
/*
pub async fn init_object_parser(client: &'static Client,object: Object, page_start: u8, page_end: u8, tx: mpsc::Sender<i32>){
    match object {
        Object::Url(urls) => {
            let mut set = JoinSet::new();
            for url in urls {
                let tx_clone = tx.clone();
                set.spawn(url_parser(client,url, page_start, page_end, tx_clone));
                //url_parser(client,url, page_start, page_end, tx_clone).await;
                
            }
            while let Some(_) = set.join_next().await {}
        },
        Object::Bvid => todo!()
    }
    /* 可以没有自动释放 */
    /* 必须释放，否则 res_selector 会一直阻塞 */
    //drop(tx);
}





struct bctp {
    bvid: String, 
    cid: String,
    titile: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response <T>{
    code: i32,
    message: String,
    ttl: i32,
    data: T,
}

/* page_start == 0 default page_end == 0 标识最大 */
async fn url_parser(client: &Client,url: String, mut page_start: u8, mut page_end: u8, tx: mpsc::Sender<i32>) {
    /* bvid cid title page
     *
     * */

    let url = Url::parse(&url).expect("Failed to parse URL");
    let mut segments = url.path_segments().ok_or_else(|| "cannot be base").unwrap();
    assert_eq!(segments.next(), Some("video"));
    let bvid = match segments.next() {
        Some(x) => x,
        None => {
            println!("no bvid");
            return
        }
    };
    if page_start == 0 {
        for (key, value) in url.query_pairs(){
            if key == "p" {
                page_end = match value.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("{}", value);
                        eprintln!("无法将字符串转换为数字");
                        return
                    }
                };
                page_start = page_end;
                break;
            }
        }
        if page_start == 0 {
            page_start = 1;
            page_end = 1;
        }
    }
    let mut url = Url::parse("https://api.bilibili.com/x/web-interface/view").expect("Failed to parse URL");
    
    url.query_pairs_mut()
        .append_pair("bvid", &bvid);
    let response = client.get(&url.to_string()).send().await.unwrap();

    {
        #[derive(Deserialize,Serialize, Debug)]
        struct Page {
            cid: i32,
        }

        #[derive(Deserialize,Serialize, Debug)]
        struct Data {
            bvid: String,
            videos: i32,
            title: String,
            cid: i32,
            pages: Vec<Page>,
        }

        let response:Response<Data>= response.json().await.unwrap();
        println!("{:#?}", response.data);

    }










    //let response:Response<Data>= response.json().unwrap();
    //println!("{:#?}", response.data);
    /*
    let response = client.get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate").send().await.unwrap();
    if !response.status().is_success() {
        return
    }

    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        url: String,
        qrcode_key: String,
    }
    let response:Response<Data>= response.json().await.unwrap();
    println!("{:?}", response);
    */








    /*
    let urls = vec!["https://www.baidu.com", "https://www.sougou.com"];
    let bodies = future::join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    })).await;
    */





    println!("{}", page_end);
}

async fn get_bvid(url: String) {

    todo!();
}
*/

