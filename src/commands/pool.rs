use errors::pool::PoolError;

use services::pool::PoolService;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


pub enum PoolCommand {
    Create(String, // name
           Option<String>, // config
           Box<Fn(Result<(), PoolError>) + Send>),
    Delete(String, // name
           Box<Fn(Result<(), PoolError>) + Send>),
    Open(String, // name
         Option<String>, // config
         Box<Fn(Result<i32, PoolError>) + Send>),
    OpenAck(i32, // cmd id
            Result<i32 /* pool handle */, PoolError>),
    Close(i32, // pool handle
          Box<Fn(Result<(), PoolError>) + Send>),
    //TODO unify type of result for open/close/refresh? (Fn(Result<i32, PoolError>))
    CloseAck(i32,
             Result<(), PoolError>),
    Refresh(i32, // pool handle
            Box<Fn(Result<(), PoolError>) + Send>),
    RefreshAck(i32,
               Result<(), PoolError>),
}

pub struct PoolCommandExecutor {
    pool_service: Rc<PoolService>,
    pending_cbs_opening: RefCell<HashMap<i32, Box<Fn(Result<i32, PoolError>)>>>,
    //TODO unify type of cb for open/close/refresh or use separate HashMaps
    //pending_cbs_close_refresh: RefCell<HashMap<i32, Box<Fn(Result<(), PoolError>)>>>,
}

impl PoolCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>) -> PoolCommandExecutor {
        PoolCommandExecutor {
            pool_service: pool_service,
            pending_cbs_opening: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: PoolCommand) {
        match command {
            PoolCommand::Create(name, config, cb) => {
                info!(target: "pool_command_executor", "Create command received");
                self.create(&name, config.as_ref().map(String::as_str), cb);
            },
            PoolCommand::Delete(name, cb) => {
                info!(target: "pool_command_executor", "Delete command received");
                self.delete(&name, cb);
            },
            PoolCommand::Open(name, config, cb) => {
                info!(target: "pool_command_executor", "Open command received");
                self.open(&name, config.as_ref().map(String::as_str), cb);
            },
            PoolCommand::OpenAck(handle, result) => {
                info!("OpenAck handle {:?}, result {:?}", handle, result);
                self.pending_cbs_opening.borrow_mut().remove(&handle)
                    .expect("Expect callback to process ack command")
                    (result);
            },
            PoolCommand::Close(handle, cb) => {
                info!(target: "pool_command_executor", "Close command received");
                self.close(handle, cb);
            },
            PoolCommand::CloseAck(handle, result) => {
                unimplemented!();
            },
            PoolCommand::Refresh(handle, cb) => {
                info!(target: "pool_command_executor", "Refresh command received");
                self.close(handle, cb);
            },
            PoolCommand::RefreshAck(handle, result) => {
                unimplemented!();
            }
        };
    }

    fn create(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn delete(&self, name: &str, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn open(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<i32, PoolError>) + Send>) {
        match self.pool_service.open(name, config.unwrap_or("TODO: default config")) { //TODO
            Err(err) => { cb(Err(err)); }
            Ok(id) => { self.pending_cbs_opening.borrow_mut().insert(id, cb); }
        };
    }

    fn close(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn refresh(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }
}