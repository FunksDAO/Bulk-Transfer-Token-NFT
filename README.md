# Transfer multiple nft to multiple owner at same time

psp34(psp34 standar contract to create, approve, transfer)
bulk_transfer(to transfer multiple nft to multiple owner at same time)

1. Upload psp34 contract
2. Upload psp22 contract
3. Mint multiple NFT
4. Upload bulk_transfer contract
5. Approve bulk_transfer contract through psp34 contract to allow transfer NFT
6. Call `bulk_nft_transfer` function and pass multiple Account where you want to transfer multiple nft
7. Increase allowance
8. Call `bulk_token_transfer` function and pass multiple account where you want to transfer multiple token
