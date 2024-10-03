const express = require('express');
const multer = require('multer');
const { WebhookClient } = require('discord.js');
const nodemailer = require('nodemailer');
const path = require('path');

const app = express();
const PORT = 3000;

// Configuração do Discord Webhook
const webhookClient = new WebhookClient({ url: 'https://discordapp.com/api/webhooks/1291407832469475471/_rh7TLrfjohsWDN_uEVucpLg7Eg6one9Dah6yCXctsbir235i4MjX9iDncRT9j0WkD9_' });

// Configuração do multer para upload de arquivos
const upload = multer({
    dest: 'uploads/',
    limits: { fileSize: 2 * 1024 * 1024 } // Limite de 2MB
});

// Rota para servir os arquivos estáticos (HTML, CSS, JS)
app.use(express.static('public'));

// Rota para processar o formulário
app.post('/submit-purchase', upload.single('comprovante'), (req, res) => {
    const { discord, email, purchaseType, plan, mysql, payment } = req.body;
    const comprovante = req.file;

    // Envia log para o Discord
    webhookClient.send({
        content: `Nova Compra!\n\n**Discord:** ${discord}\n**Email:** ${email}\n**Tipo de Compra:** ${purchaseType}\n**Plano:** ${plan}\n**MySQL:** ${mysql}\n**Pagamento:** ${payment}\n**Comprovante:** ${comprovante ? comprovante.filename : 'Nenhum'}`
    });

    // Opcional: Configurar envio de e-mails usando nodemailer

    res.json({ message: 'Compra realizada com sucesso!' });
});

app.listen(PORT, () => {
    console.log(`Servidor rodando na porta ${PORT}`);
});
