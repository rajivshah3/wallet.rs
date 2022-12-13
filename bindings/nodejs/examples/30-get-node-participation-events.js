/**
 * This example gets the participation events from node
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();

        const events = await manager.getNodeParticipationEvents(0)
        
        console.log('Node Participation Events:', events)
        
        process.exit(0)
    } catch (error) {
        console.log('Error: ', error);
        process.exit(1);
    }
}

run();
