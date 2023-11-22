//! Generated types support documentation
#![deny(missing_docs)]
use anyhow::Ok;
use schemars::JsonSchema;
use futures::{pin_mut, TryStreamExt};

use serde::{Deserialize, Serialize};
use tracing::*;
use apiexts::CustomResourceDefinition;
use k8s_openapi::{apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts, serde, api::core::v1::Node};

use kube::{
    api::{Api, Patch, PatchParams, ResourceExt,ListParams},
    runtime::wait::{await_condition, conditions},
    runtime::{watcher, WatchStreamExt},
    Client, CustomResource, CustomResourceExt,
};

// InventoryCRD
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "stackzoo.io", version = "v1", kind = "Inventory", namespaced)]
#[kube(status = "InventoryStatus")]
#[kube(scale = r#"{"specReplicasPath":".spec.replicas", "statusReplicasPath":".status.replicas"}"#)]
struct InventorySpec {
    pub name: String,
    pub nodes: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
struct InventoryStatus {
    pub is_bad: bool,
}

const crdName : &str = "inventories.stackzoo.io";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    let ssapply = PatchParams::apply("inventory_apply").force();
    // 0. Ensure the CRD is installed, could do this once
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    info!("Creating crd: {}", serde_yaml::to_string(&Inventory::crd())?);
    crds.patch(crdName, &ssapply, &Patch::Apply(Inventory::crd()))
        .await?;

    info!("Waiting for the api-server to accept the CRD");
    let establish = await_condition(crds, crdName, conditions::is_crd_established());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await?;

    // Let's get the current node inventory
    let nodes: Api<Node> = Api::all(client.clone());
    // New client copy to inject our resource
    let inventories: Api<Inventory> = Api::default_namespaced(client.clone());

    let spec = create_spec(nodes).await;

    let tt = inventories.patch("default", &ssapply,
     &Patch::Apply(&Inventory::new("default", spec))).await?;

    info!("Applied 1 {}: {:?}", tt.name_any(), tt.spec);


    // watch the inventory resources
    let obs = watcher(inventories, ListParams::default()).applied_objects();
    pin_mut!(obs);
    while let Some(o) = obs.try_next().await? {
        match o {
            Node => {
                let nodes: Api<Node> = Api::all(client.clone());
                let spec = create_spec(nodes.clone()).await;
                let inventories: Api<Inventory> = Api::default_namespaced(client.clone());

                let tt = inventories.patch("default",
                 &ssapply,
                 &Patch::Apply(&Inventory::new("default", 
                 spec))).await?;
            }


        }
    }

    Ok(())
}
async fn create_spec(nodes: Api<Node>) -> InventorySpec {

    let node_list = nodes.list(&ListParams::default()).await.unwrap();
    let mut node_names = Vec::new();
    for node in node_list {
        node_names.push(node.metadata.name.unwrap());
    }
    return InventorySpec {
        name: "default".to_string(),
        nodes: node_names,
    };
}