use crate::{config, models};
use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, error, info};
use tiny_http::{Header, Response, Server};
use rand::{distributions::Alphanumeric, Rng};

const JOB_ID_HEADER_NAME: &str = "Process-ID";
pub(crate) struct WorkflowServer{
    config: config::Config,
    server: Option<Server>,
    answer_cache: HashMap<String, String>,
}

impl WorkflowServer {
    

    pub fn new(config: config::Config) -> WorkflowServer {
        WorkflowServer {
            config,
            server: None,
            answer_cache: HashMap::new(),
        }
    }

    pub fn start_server(&mut self) -> Result<bool, &'static str> {
        if self.server.is_some() {
            return Err("Server is already running");
        }
        let server = Server::http(self.config.server.clone()).unwrap();
        info!("Server listening on http://{}", self.config.server);
        self.server = Some(server);
        Ok(true)
    }

/// Handles incoming HTTP requests and processes them based on the defined process groups.
///
/// # Parameters
///
/// * `&mut self` - A mutable reference to the `WorkflowServer` instance.
///
/// # Return
///
/// This function does not return a value. It is an asynchronous function that handles incoming HTTP requests.
pub async fn handle_request(&mut self) {
    // 获取服务器实例的引用
    let server = if let Some(server) = &self.server {
        server
    } else {
        error!("Server is not running.");
        return;
    };

    loop {
        let mut request = server.recv().unwrap();

        let process_group = self.config.process_group.iter().find(|group| {
            request.url() == format!("/{}", group.name)
        });

        if process_group.is_none() {
            request.respond(Response::from_string("Route not found")).unwrap();
            continue;
        }

        match process_group {
            Some(group) => {
                let mut question = String::new();
                request.as_reader().read_to_string(&mut question).unwrap();
                match request.method().as_str() {
                    "POST" => {
                        debug!("Question: {}", question);
                        let job_id: String = rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(10) // Set the length of the string
                            .map(char::from)
                            .collect();
                        info!("{:?} Get Job {:?}", request.url(), job_id);
                        let mut model = models::gm_model::GModel::new(&self.config);
                        model.set_prompt(group.prompt.clone());
                        let answer = model.ask(question).await.clone();
                        self.answer_cache.insert(job_id.clone(), answer);
                        debug!("--> {:?}", self.answer_cache);
                        let mut response = Response::from_string(format!(
                            "Process group {} is running, ID: {}",
                            group.name, job_id.clone()
                        ));
                        response.add_header(Header::from_str(format!(
                            "{}: {}",
                            JOB_ID_HEADER_NAME, job_id
                        ).as_str()).unwrap());
                        request.respond(response).unwrap();
                    }
                    "GET" => {
                        debug!("Get answer for {:?}", self.answer_cache);
                        let process_id = request.headers().iter().find(|x| {
                            x.field == JOB_ID_HEADER_NAME.parse().unwrap()
                        });
                        let response = match process_id {
                            Some(id) => {
                                let answer = self.answer_cache.remove(&id.value.to_string()).unwrap();
                                Response::from_string(answer)
                            }
                            None => {
                                Response::from_string("Not Found ID")
                            }
                        };
                        request.respond(response).unwrap();
                    }
                    _ => {
                        request.respond(Response::from_string("Unsupported method")).unwrap();
                    }
                }
            },
            None => {
                let response = Response::from_string("Route not found");
                request.respond(response).unwrap();
            }
        }
    }
}
}

