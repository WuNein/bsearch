var init, {cut} = require('./rustJieba.js')
console.log(init)
async function main() {
    await init('./rustJieba_bg.wasm')
    let res = cut("我在这里")
    console.log(res)
}
main()