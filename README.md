# 🐻‍❄️ Noelware.Telemetry
> *Project agnostic telemetry server made for Noelware projects, made in C# using ASP.Net + Cassandra*

## Why are you building this? I'm going to lose trust!
I know, startup companies and evil tech corporations want to steal all of your data and sell it to
advertisers and do even more evil things, but that's not the purpose of this project and **Noelware** itself.

Noelware's plan is create high quality, FOSS software for the average user and infrastructure utilities for
DevOps teams, original right? Probably, I don't know!

You could also be asking: "Noel, is this really needed?"

The answer to that is a simple **yes**.

It is benefitial to have this project for the Noelware team because we'll send:

- anonymized data usage (from CLI applications, backends, and frontends)
- error reports (ones to prioritize the most)

If you don't want to opt into telemetry, you can always disable it (we will have an option to disable it!) and
it is always disabled when you run our software, so that's on you!

If you still don't trust me, that's alright! You don't need to use our stuff if you think we'll turn into
a evil tech corporation and sell all your data for profit! You can read up on our [ToS](https://noelware.org/tos) and [Privacy Policy](https://noelware.org/privacy) for more information.

You can also read our documentation [here](https://docs.noelware.org/telemetry)!

## Installation
Since this is a platform agnostic server for any project, you can use it on yours that is 100% anonymized and
only stores what is needed.

You can install it on the following:

- using the Noelware [Helm Chart](https://charts.noelware.org/noelware/telemetry-server) ([#](#helm-chart))
- using the **Docker Image** ([#](#docker))
- locally on your system using **Git** ([#](#locally-with-git))

### Helm Chart
This is the easiest way to bootstrap **telemetry-server** on your Kubernetes cluster, we require you to have
Kubernetes **>=1.22** and Helm **3**!

```sh
# 1. Pull from the helm chart server
$ helm repo add noelware https://charts.noelware.org/noelware

# 2. Install!
$ helm install <my-release> noelware/telemetry-server
```

### Docker
Soon!

### Locally with Git
Soon!

## Contributing
Soon!

## Configuration
Soon!

## License
**telemetry-server** is released under the **GPL-3.0** License by Noelware. Read [here](/LICENSE) for more information.
