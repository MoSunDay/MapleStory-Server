#!/bin/bash
# Launch the MapleStory server (HeavenMS v83) on JRE 11 (provides the Nashorn JS engine).
# Prereqs: MariaDB 'maplestory' DB loaded from sql/, wz/ XML data present.
cd "$(dirname "$0")"
J11=/usr/lib/jvm/java-11-openjdk-amd64
JAVA=${J11}/bin/java
# Filter the harmless "Nashorn engine is planned to be removed" deprecation notice from stderr.
exec "$JAVA" -Xmx2048m -Dwzpath=wz \
  -cp ".:dist:cores/*" net.server.Server \
  2> >(grep -v --line-buffered "Nashorn engine is planned to be removed")
