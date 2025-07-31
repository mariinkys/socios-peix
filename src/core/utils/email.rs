use crate::core::models::{cupon::Cupon, member::Member};

pub const LOGO: &[u8] = include_bytes!("../../../resources/logo.png");

pub enum EmailKind {
    Normal,
    Birthday,
    Cupon,
}

impl EmailKind {
    pub fn get_styled(
        &self,
        email_body: Option<String>,
        member: Option<Member>,
        cupon: Option<Cupon>,
    ) -> Result<String, anyhow::Error> {
        match &self {
            EmailKind::Normal => {
                let email_body = email_body.ok_or_else(|| anyhow::anyhow!("Missing email body"))?;

                Ok(format!(
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
                                <p style="font-size: 14px; color: #777; text-align: center; margin-top: 30px;">
                                    ¡Gracias por confiar en nosotros!<br/>
                                    El equipo del Hotel Casa Peix.
                                </p>
                            </div>
                        </body>
                    </html>
                    "#
                ))
            }
            EmailKind::Birthday => {
                let member = member.ok_or_else(|| anyhow::anyhow!("Missing member body"))?;

                Ok(format!(
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
                                <p style="font-size: 14px; color: #777; margin-top: 30px;">
                                    Con nuestros mejores deseos,<br/>
                                    El equipo del Hotel Casa Peix.
                                </p>
                            </div>
                        </body>
                    </html>
                    "#,
                    nombre = format_args!(
                        "{} {} {}",
                        member.name, member.surname, member.second_surname
                    )
                ))
            }
            EmailKind::Cupon => {
                let cupon = cupon.ok_or_else(|| anyhow::anyhow!("Missing cupon"))?;
                let member = member.ok_or_else(|| anyhow::anyhow!("Missing member"))?;
                let email_body = email_body.ok_or_else(|| anyhow::anyhow!("Missing email body"))?;

                Ok(format!(
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
                                    Descuento: <strong>{descuento}%</strong><br/>
                                    Válido hasta: <strong>{fecha}</strong>
                                </div>
                                <p style="font-size: 14px; color: #777; text-align: center;">
                                    ¡Reserve ahora y aproveche esta oportunidad!<br/>
                                    El equipo del Hotel Casa Peix.
                                </p>
                            </div>
                        </body>
                    </html>
                    "#,
                    nombre = member.name,
                    codigo = cupon.code,
                    descuento = cupon.description,
                    fecha = cupon
                        .expires_at
                        .map(|x| x.format("%d-%m-%Y").to_string())
                        .unwrap_or_else(|| "Error, fecha desconocida".to_string())
                ))
            }
        }
    }
}
