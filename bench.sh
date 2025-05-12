#!/bin/bash

# 定义非美国地区
REGIONS="ams bog otp eze fra gdl hkg jnb lhr mad yul bom cdg gig scl gru sin arn syd nrt yyz waw"

# 循环为每个地区设置 1 个 Machine
for region in $REGIONS; do
  echo "Scaling Machine in $region..."
  fly scale count 1 --region $region
  sleep 2
done