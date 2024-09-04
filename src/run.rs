use crate::config::ProcessGroup;
use crate::{config, models};
use futures::future::join_all;
use log::debug;

pub struct Processor {
    config: config::Config,
}

impl Processor {
    pub fn new(config: config::Config) -> Self {
        Processor {
            config
        }
    }

    pub async fn handle_group(&self, group: & ProcessGroup) -> Result<bool, &'static str> {
        debug!("handle_group {:?}", group);
        let question = String::new();
        debug!("Question: {}", question);
        let model = models::gm_model::GModel::new(&self.config);
        let response = model.ask(question).await.clone();
        debug!("{:?}", response);
        Ok(true)
    }

    pub async fn process(&self) -> Result<bool, &'static str> {
        let process_groups = self.config.process_group.clone();
        let mut async_group = vec![];
        for group in process_groups {
            debug!("{:?}", group);
           //  async_group.push(self.handle_group(&clone_group));
            async_group.push(async move {
                let _ = self.handle_group(&group).await;
            });
        }

        join_all(async_group).await;
        Ok(true)
    }


}