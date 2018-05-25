/**
 * Extracts the ABIs from the Truffle artifacts in '../build/contracts' and
 * puts them in '../build/abi/'... MUST RUN FROM THE ROOT DIRECTORY ex. node ./scripts/extractABIs.js
 */
const fs = require('fs')

const main = async () => {
    const files = fs.readdirSync('./build/contracts')
    const ABIs = files.map(file => {
        const json = JSON.parse(fs.readFileSync(`./build/contracts/${file}`))
        return {
            filename: file,
            abi: json.abi,
        }
    })

    const target = './build/abis/'
    fs.mkdirSync(target)

    ABIs.map(abiObj => {
        fs.writeFileSync(target +  abiObj.filename.slice(0, -4).concat('abi'), JSON.stringify(abiObj.abi))
    })
}

try {
    main()
} catch (e) {
    console.error(e)
}