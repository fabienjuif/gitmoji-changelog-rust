// this comes from: https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json
const https = require('https')

https.get('https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json', (resp) => {
  let data = ''
  resp.on('data', (chunk) => { data += chunk })
  resp.on('end', () => {
    console.log(JSON.parse(data).gitmojis.map(({ emoji, code }) => `m.insert("${code.replace(/:/g, '')}", "${emoji}");`).join('\n'))
  })
}).on("error", (err) => { console.log("Error: " + err.message) })
