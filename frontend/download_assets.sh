#!/bin/bash

url="https://deckofcardsapi.com/static/img"

families=("C" "D" "S" "H")
values=("2" "3" "4" "5" "6" "7" "8" "9" "0" "J" "Q" "K" "A")

for family in "${families[@]}"; do
  for value in "${values[@]}"; do
        curl $url/$value$family.png -o ./src/assets/cards/$value$family.png
  done
done

curl $url/back.png -o ./src/assets/cards/back.png
