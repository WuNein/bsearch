var init, {cut} = require('./rustJieba.js')
console.log(init)
async function main() {
    await init('./rustJieba_bg.wasm')
    let res = cut("ζε¨θΏι")
    console.log(res)
}
main()