const express = require('express')
const app = express()
const port = 5500



app.use(express.static('public'))
app.use(express.json())

app.get((req,res) => {
    res.status(200).json({ info: 'preset text'})
})

app.post('/',(req,res) => {
    const { parcel } = req.body
    if(!parcel) {
        return res.status(400).send({ status: 'failed' })
    }
    res.status(200).send({ status: 'received' })
})