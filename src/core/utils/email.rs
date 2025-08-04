use crate::core::models::{cupon::Cupon, member::Member};

pub const LOGO: &[u8] = include_bytes!("../../../resources/logo.png");

pub fn get_normal_styled(email_body: String) -> String {
    format!(
        r#"
        <html>
            <body style="font-family: Arial, sans-serif; background-color: #f9f9f9; color: #333;">
                <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px; padding: 20px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
                    <div style="text-align: center; margin-bottom: 20px;">
                        <img src="cid:logo.png" alt="Hotel Logo" style="width: 250px; height: auto;"/>
                    </div>
                    <p style="font-size: 16px; line-height: 1.6;">
                        {email_body}
                    </p>
                    <p style="font-size: 14px; color: #5b5b5b; text-align: center; margin-top: 30px;">
                        ¡Gracias por confiar en nosotros!<br/>
                        El equipo del Hotel Casa Peix.
                    </p>
                    <p style="font-size: 12px; color: #777; text-align: center;">
                        (Puede darse de baja en cualquier momento escribiendo un correo a infohotelcasapeix@gmail.com)
                    </p>
                </div>
            </body>
        </html>
        "#
    )
}

pub fn get_birthday_styled(member: &Member) -> String {
    format!(
        r#"
        <html>
            <body style="font-family: Arial, sans-serif; background-color: #fff7e6; color: #333;">
                <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px; padding: 30px; box-shadow: 0 2px 6px rgba(0,0,0,0.1); text-align: center;">
                    <img src="cid:logo.png" alt="Hotel Logo" style="width: 250px; height: auto;"/>
                    <h1 style="color: #d35400;">¡Feliz Cumpleaños, {nombre}!</h1>
                    <p style="font-size: 16px; line-height: 1.6;">
                        Esperamos que este día esté lleno de alegría, momentos especiales y felicidad.
                    </p>
                    <p style="font-size: 16px; line-height: 1.6;">
                        Como muestra de nuestro cariño, queremos recordarle que siempre será bienvenido(a) en nuestro hotel.
                    </p>
                    <p style="font-size: 14px; color: #5b5b5b; margin-top: 30px;">
                        Con nuestros mejores deseos,<br/>
                        El equipo del Hotel Casa Peix.
                    </p>
                    <p style="font-size: 12px; color: #777; text-align: center;">
                        (Puede darse de baja en cualquier momento escribiendo un correo a infohotelcasapeix@gmail.com)
                    </p>
                </div>
            </body>
        </html>
        "#,
        nombre = format_args!(
            "{} {} {}",
            member.name, member.surname, member.second_surname
        )
    )
}

pub fn get_cupon_styled(email_body: String, member: &Member, cupon: &Cupon) -> String {
    format!(
        r#"
        <html>
            <body style="font-family: Arial, sans-serif; background-color: #f4f9ff; color: #333;">
                <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px; padding: 20px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
                    <div style="text-align: center; margin-bottom: 20px;">
                        <img src="cid:logo.png" alt="Hotel Logo" style="width: 250px; height: auto;"/>
                    </div>
                    <h2 style="color: #2980b9; text-align: center;">¡Estimado(a) {nombre}, tenemos algo especial para usted!</h2>
                    <p style="font-size: 16px; line-height: 1.6; text-align: center;">
                        {email_body}
                    </p>
                    <div style="background: #eaf4ff; border: 2px dashed #2980b9; padding: 15px; text-align: center; font-size: 18px; margin: 20px 0;">
                        Código de Cupón: <strong>{codigo}</strong><br/>
                        Descripción: <strong>{descripcion}</strong><br/>
                        Válido hasta: <strong>{fecha}</strong>
                    </div>
                    <p style="font-size: 14px; color: #5b5b5b; text-align: center;">
                        ¡Aproveche esta oportunidad!<br/>
                        El equipo del Hotel Casa Peix.
                    </p>
                    <p style="font-size: 12px; color: #777; text-align: center;">
                        (Puede darse de baja en cualquier momento escribiendo un correo a infohotelcasapeix@gmail.com)
                    </p>
                    <p style="font-size: 10px; color: #5b5b5b; text-align: center;">
                        El establecimiento se reserva el derecho a no admitir cualquier cupón independientemente de su fecha de caducidad
                    </p>
                </div>
            </body>
        </html>
        "#,
        nombre = member.name,
        codigo = cupon.code,
        descripcion = cupon.description,
        fecha = cupon
            .expires_at
            .map(|x| x.format("%d-%m-%Y").to_string())
            .unwrap_or_else(|| "Error, fecha desconocida".to_string())
    )
}
