# Introduction

Bayou is an in development fediverse server written in rust with the goal of being a simple and reliable server with decent performance

Bayou is currently a work in progress and the migrations will be in a constant state of change until the first alpha. Because of this it sould not be run in production and it will have continuous major breaking changes. 

Bayou is split into [bayou_server](/server) which will provide a full featured activitypub server and [bayou_protocol](/protocol) which provides bayou types as well as cryptography and protocol logic which can be enabled via features. Eventually a custom frontend for Bayou will be written using yew and the bayou_protocol types

Bayou intends to impliment the activitypub protocol as well as a secondary effort in implimenting the versia protocol. Bayou may impliment other protocols in the future but mastodon-activitypub is the core focus

# Environment Setup

for setting up your environment to run, check [environment setup](environment_setup.md)
