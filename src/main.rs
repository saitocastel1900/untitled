use std::ops::Range;
use std::result;

fn main() {

    let  s1=("HelloWorld");
    println!("{}",s1);
    let n=0;

    //タプルを使ってみる
    let mut mus =(1, "2");

    println!("{}",mus.0);
    mus.0=20;
    mus.1="22";
    println!("{}",mus.0);

    //str ="こんにちわ　Rust";
    //println!("{}",str);

    //配列を使ってみる
    let mut a: [i32;3]=[0,1,2];
    for  i in a {
       println!("{}",i);
    }

    //構造体を実装してみる
    struct Person{
        nsmr:String,
        age:u32,
    }

    let p=Person{
        nsmr:String::from("John"),
        age:8,
    };

    //列挙型を使ってみる
    enum Event{
        Quit,
        Save,
        Delete,
    }

    let event1=Event::Quit;
    let  event2=Event::Save;

    //Optionはデータが存在しているか否かを表現する
    enum Optin<T>{
        None,
        Some(T),
    }

    //データが存在しない場合はNone
    let result:Result<i32,String>=Ok(200);

    match  result{
        Ok(core)=>println!("code:{}",core),
        Err(err)=>println!("Err:{}",err),
    }

    //Vector型を使ってみる
    let v1=vec![1,2,3,4,5];
    //0を5個埋める
    let v2=vec![0;5];
    for v in v1 {
        println!("{}",v)
    }

    //rustでは通常値を後から変更はできない
    let mut mutvalue=0;
    mutvalue=20;
    println!("{}",mutvalue);

    //rustではこのように式として使う
    let number=1;
    let result=if 0<=number{
        number
    }else {
        -number
    };

    //loopはずっと繰り返す
    let mut count=0;

    let result= loop {
        count+=1;
        if count==10{
            //戻り値としても使用できる
      break count;
        }
    };

    //マッチング(switch)を使ってみる
    //符号あり整数の32ビットサイズ変数
    let i:i32=1;
    match i {
     1=>println!("1"),
        2=>println!("2"),
        3=>println!("3"),
        //ワイルド記法でも指定できる
        _=>println!("No Dokonimoarimase---nn!"),
    }

    //構造体にメソッドを追加してみる
    struct Character{
    name:String,
        gender:i32,
    }
    impl Character{

        fn Log(&self){
        println!("name:{} Gender(male=0 or female=1):{}",self.name,self.gender)
        }
    }

    let man=Character{name:String::from("Taro"), gender:20};
    man.Log()
}
