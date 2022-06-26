const COST_PER_BATCH = 5
const COST_PER_WAVE = 10
const COST_PER_AISLE = 5
const COST_PER_WAREHOUSE = 10
const MAX_BATCH_SIZE = 10000
const MAX_WAVE_SIZE = 250
const input = require(process.argv[2] || "./instances/instance4.json")
const output = require(process.argv[3] || "./solutions/output4.json")

const locations = new Map()
for (const location of input.ArticleLocations) {
    locations.set(location.ArticleId, location)
}

const batchesMap = new Map()
for (const batch of output.Batches) {
    batchesMap.set(batch.BatchId, batch)
}

// validate all waves are small enough
for (const wave of output.Waves) {
    const waveSize = wave.BatchIds.reduce((acc, batchId) => {
        const batch = batchesMap.get(batchId)
        const numArticles = batch.Items.length
        return acc + numArticles
    }, 0)

    if (waveSize > MAX_WAVE_SIZE) {
        console.log(`Wave ${wave.WaveId} contains ${waveSize} articles!`)
        process.exit(1)
    }
}

// validate all batches are small enough
for (const batch of output.Batches) {
    const batchSize = batch.Items.reduce((acc, item) => {
        const articleVolume = input.Articles[item.ArticleId].Volume
        return acc + articleVolume
    }, 0)

    if (batchSize > MAX_BATCH_SIZE) {
        console.log(`Batch ${batch.BatchId} has volume ${batchSize}!`)
        process.exit(1)
    }
}

// validarte all orders are fulfilled
for (const order of input.Orders) {
    const waves = output.Waves.filter(w => w.OrderIds.includes(order.OrderId))
    if (waves.length !== 1) {
        console.log(`Order ${order.OrderId} is found in ${waves.length} waves!`)
        process.exit(1)
    }

    const wave = waves[0]
    const batches = wave.BatchIds.map(bId => batchesMap.get(bId))
    const articleIds = new Set(batches.flatMap(b => b.Items).map(item => item.ArticleId))

    for (const articleId of order.ArticleIds) {
        if (!articleIds.has(articleId)) {
            console.log(`Article ${articleId} missing from order ${order.OrderId}`)
            process.exit(1)
        }
    }
}

const tourCost = output.Batches.reduce((acc, batch) => {
    const articleLocations = batch.Items.map(item => locations.get(item.ArticleId))
    const warehouses = new Set(articleLocations.map(l => l.Warehouse))
    const aisles = new Set(articleLocations.map(l => "" + l.Warehouse + l.Aisle))

    const cost = warehouses.size * COST_PER_WAREHOUSE + aisles.size * COST_PER_AISLE
    return acc + cost
}, 0)

const restCost = output.Waves.length * COST_PER_WAVE + output.Batches.length * COST_PER_BATCH

console.dir({
    tourCost,
    restCost,
    totalCost: tourCost + restCost
})