# KUBERNETES RUST OPERATOR POC 🦀 ☸

## Abstract

This repo contains a straightforward *proof of concept* of a simple Kubernetes operator made with [kube-rs](https://github.com/kube-rs/kube).  
This operator does nothing but define a CRD called `inventories.stackzoo.io` that basically gather nodes information (names) via the kubernetes api.  


> **Warning**  
> This is just a PoC, do not use in production!

## Prerequisites

- [KinD](https://kind.sigs.k8s.io/)  
- [Rust](https://www.rust-lang.org/it)  

## Instructions to Test

Spin up a 4 node local cluster with `KinD`:  
```console
kind create cluster --config kind-cluster-config.yaml
```  
Output:  
```console
Creating cluster "operator-cluster" ...
 ✓ Ensuring node image (kindest/node:v1.27.3) 🖼 
 ✓ Preparing nodes 📦 📦 📦 📦  
 ✓ Writing configuration 📜 
 ✓ Starting control-plane 🕹️ 
 ✓ Installing CNI 🔌 
 ✓ Installing StorageClass 💾 
 ✓ Joining worker nodes 🚜 
Set kubectl context to "kind-operator-cluster"
You can now use your cluster with:

kubectl cluster-info --context kind-operator-cluster

Thanks for using kind! 😊
```  

Now launch the operator with the *cargo run* command:  
```console
cargo run

2023-11-22T09:37:48.187293Z  INFO k8s_rust_operator_poc: Creating crd: ---
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: inventories.stackzoo.io
spec:
  group: stackzoo.io
  names:
    categories: []
    kind: Inventory
    plural: inventories
    shortNames: []
    singular: inventory
  scope: Namespaced
  versions:
    - additionalPrinterColumns: []
      name: v1
      schema:
        openAPIV3Schema:
          description: "Auto-generated derived type for InventorySpec via `CustomResource`"
          properties:
            spec:
              properties:
                name:
                  type: string
                nodes:
                  items:
                    type: string
                  type: array
              required:
                - name
                - nodes
              type: object
            status:
              nullable: true
              properties:
                is_bad:
                  type: boolean
              required:
                - is_bad
              type: object
          required:
            - spec
          title: Inventory
          type: object
      served: true
      storage: true
      subresources:
        scale:
          specReplicasPath: ".spec.replicas"
          statusReplicasPath: ".status.replicas"
        status: {}

2023-11-22T09:37:48.253748Z  INFO k8s_rust_operator_poc: Waiting for the api-server to accept the CRD
2023-11-22T09:37:48.353980Z  INFO k8s_rust_operator_poc: Applied 1 default: InventorySpec { name: "default", nodes: ["operator-cluster-control-plane", "operator-cluster-worker", "operator-cluster-worker2", "operator-cluster-worker3"] }
```  
Open a new terminal session and check your cluster CRDs:  
```console
kubectl get crds

NAME                      CREATED AT
inventories.stackzoo.io   2023-11-22T09:37:48Z
```  
Let's see what instances of inventories.stackzoo.io we have on our cluster:  
```console
kubectl get inventories.stackzoo.io

NAME      AGE
default   2m59s
```   
Now let's retrieve that CR with the `kubectl get inventories.stackzoo.io default -o yaml` command  :  
```yaml
apiVersion: stackzoo.io/v1
kind: Inventory
metadata:
  creationTimestamp: "2023-11-22T09:37:48Z"
  generation: 1
  name: default
  namespace: default
  resourceVersion: "836"
  uid: 6add7ef4-fb3b-49ed-b8df-de704abbf923
spec:
  name: default
  nodes:
  - operator-cluster-control-plane
  - operator-cluster-worker
  - operator-cluster-worker2
  - operator-cluster-worker3
```  
As we can see we find our cluster nodes listed under the `spec/nodes` property.  

## Next Steps
For a slightly more intricate example, please refer to [this](https://github.com/Pscheidl/rust-kubernetes-operator-example) repository.  









