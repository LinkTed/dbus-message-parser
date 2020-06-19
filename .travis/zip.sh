#!/bin/bash

prefix="target/debug/deps"
executables=("$prefix/dbus_message_parser-*.gc*"
	     "$prefix/header-*.gc*"
	     "$prefix/mod-*.gc*")

zip -0 ccov.zip ${executables[@]}
