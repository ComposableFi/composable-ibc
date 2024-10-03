POSTFIX="local"

#cp ~/work/cf_guest_cw_2.wasm ics10_grandpa_cw_$POSTFIX.wasm
cp target/wasm32-unknown-unknown/release/ics10_grandpa_cw.wasm ics10_grandpa_cw_$POSTFIX.wasm
wasm-opt ics10_grandpa_cw_$POSTFIX.wasm -o ics10_grandpa_cw_$POSTFIX.opt.wasm -Os
#base64 -i ics10_grandpa_cw_$POSTFIX.opt.wasm -o contract-data-$POSTFIX.txt
#perl -0777 -pe 'BEGIN{ local $/; open my $f, "<", "contract-data-'"$POSTFIX"'.txt"; $replacement = <$f>; $replacement =~ s/^\s+|\s+$//g ; close $f } s/<CODE>/$replacement/gs' msg_contract.template.json > msg_contract.json
#cp msg_contract.json ../composable-testnet

echo "Code id:"
# sha2 -256 -q ics10_grandpa_cw_$POSTFIX.opt.wasm
