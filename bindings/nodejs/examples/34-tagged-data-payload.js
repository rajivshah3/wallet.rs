/**
 * This example will send a transaction with a tagged data payload
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();
        const account = await manager.getAccount('Alice');

        await account.sync();

        //TODO: Replace with the address of your choice!
        const address =
            'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0';
        const amount = '1000000';

        const response = await account.sendAmount([
            {
                address,
                amount,
            },
        ],
            {
                taggedDataPayload: {
                    type: 5,
                    // 'Stardust' hex encoded
                    tag: "0x5374617264757374", data: "0x5374617264757374"
                }
            });

        console.log(response);

        console.log(
            `Check your block on http://localhost:14265/api/core/v2/blocks/${response.blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}


run();
