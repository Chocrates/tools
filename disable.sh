#!/bin/bash

xinput --disable $(xinput --list | grep DLL075 | cut -f2 -d$'\t' | cut -f2 -d'=')
