use std::error::Error;
use std::fmt;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

use std::env::set_var;

#[allow(unused_imports)]
use thirtyfour::ChromiumLikeCapabilities;
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

#[allow(dead_code)]
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");

    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Warn);
           // let _ti#[warn(dead_code)]mestamp = buf.timestamp();
            writeln!(
                buf,
                // FROM HERE
                // https://docs.rs/env_logger/latest/src/custom_format/custom_format.rs.html#35
                "{}:{}  {warn_style}{}{warn_style:#} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();

    error!("RUST_LOG maybe NOT enable");
    error!("Used: => RUST_LOG=info < prg >");

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let _ = rt.block_on(run());

    info!("env_logger: ended");
    process::exit(0);
}

async fn run() -> Result<(), Box<dyn Error>> {
    let _place: &str = "Place";

    // path_to().await?;
    download_file().await?;

    Ok(())
}

async fn download_file() -> Result<(), Box<dyn Error>> {
    debug!("call init_driver()");

    // double
    let mut _driver = init_driver().await?;
    let mut _ref_driver = &mut _driver;

    let _open: String = String::from("open");
    let _call: String = String::from("call");
    let _close: String = String::from("close");

    debug!("execute_command  _cmd => _open ");
    let _execute_command_result = execute_command(&_ref_driver,&_open).await;
    debug!("execute_command finish _cmd => _open ");

    // let _ = match _execute_command_result {
    //     //everything is fine
    //     Ok(()) => {
    //         debug!("Ok");
    //         ()
    //     }
    //     Err(_e) => {
    //         return Err(Box::new(MyError("Error _execute_command => {_e}".to_string())).into())
    //     }
    // };

    let _ = match _execute_command_result {
        Ok(_web_element) => {
            info!(r#"ACTION_BROWSER_CLOSE => Ok"#);
        }
        Err(_e) => {
            error!(r#"ACTION_BROWSER_CLOSE => Err {_e}"#);
            return Err(Box::new(MyError("Error _execute_command => {_e}".to_string())).into())
        }
    };
    debug!("execute_command  _cmd => _open ");
    //wait
    // debug!("wait 10 sec");
    // let _ = wait_seconds_of_browser(3);

    let _ = wait_seconds_of_browser(_ref_driver,5).await; 

    debug!("execute_command  _cmd => _close ");
    let _execute_command_result = execute_command(&_ref_driver,&_close).await;

    let _ = match _execute_command_result {
        //everything is fine
        Ok(()) => (),
        Err(_e) => {
            return Err(Box::new(MyError("Error _execute_command => {_e}".to_string())).into())
        }
    };
    Ok(())
}




async fn execute_command(_ref_driver:&WebDriver,cmd: &String) -> Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", cmd);
    
    debug!("execute_command  _cmd => {}", cmd);


   // let _driver = init_driver().await?;

    // } else
    if cmd == "close" {
        // debug!("wait 3 sec");
        // let _ = wait_seconds_of_browser(_driver.clone(), 3);

        debug!("execute_command  _cmd => {}", cmd);
        let result_close_browser = close_browser(_ref_driver.clone()).await;
        let _ = match result_close_browser {
            Ok(_web_element) => {
                info!(r#"ACTION_BROWSER_CLOSE => Ok"#);
            }
            Err(_e) => {
                error!(r#"ACTION_BROWSER_CLOSE => Err {_e}"#);
            }
        };

        // let _result_init_driver = init_driver();
    } else if cmd == "open" {
        debug!("execute_command  cmd => {}", cmd);
    } else {
        info!("Opps!!! Command NOT FOUND {}", cmd);
    }
    debug!("finished => execute_command -> {}", cmd);

    Ok(())
}

async fn close_browser(_driver: WebDriver) -> Result<(), Box<dyn Error>> {
    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

async fn init_driver() -> Result<WebDriver, WebDriverError> {
    info!("init_driver - start");

    let mut _caps = DesiredCapabilities::chrome();

    _caps.add_arg("--remote-debugging-pipe")?;
    _caps.add_arg("--no-sandbox")?;

    let _driver_result = WebDriver::new("http://localhost:9515", _caps).await;

    // let result = WebDriver::new("http://localhost:4444/wd/hub", &caps).await;
    let _driver = match _driver_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    _driver.maximize_window().await?;
    info!("init_driver - end");
    Ok(_driver)
}

// async fn wait_seconds_of_browser(waiting_period: u64) -> Result<(), Box<dyn Error>> {
//     // debug!("wait for page completed load => wait for status from chrome driver");
//     // debug!("driver=> {:?}", _driver.status().await?);
//     debug!("Thread sleep for {} seconds", waiting_period);
//     thread::sleep(Duration::from_secs(waiting_period));
//     Ok(())
// }

async fn wait_seconds_of_browser(
    _driver: &mut WebDriver,
    waiting_period: u64,
) -> Result<(), Box<dyn Error>> {
    debug!("wait for page completed load => wait for status from chrome driver");
    debug!("driver=> {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}
