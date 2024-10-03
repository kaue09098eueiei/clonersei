const transporter = nodemailer.createTransport({
    service: 'gmail',
    auth: {
        user: 'seu-email@gmail.com',
        pass: 'sua-senha'
    }
});

const mailOptions = {
    from: 'seu-email@gmail.com',
    to: email,
    subject: 'Confirmação de Compra',
    text: `Olá ${discord}, sua compra foi realizada com sucesso!`
};

transporter.sendMail(mailOptions, (error, info) => {
    if (error) {
        console.log(error);
    } else {
        console.log('Email enviado: ' + info.response);
    }
});
