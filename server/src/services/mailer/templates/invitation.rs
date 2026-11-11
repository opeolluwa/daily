use askama::Template;

#[derive(Template)]
#[template(path = "invitation.html")]
pub struct InvitationTemplate {
    pub signup_link: String,
    pub invitee_name: String,
    pub inviter_name: String,
    pub role_name: String,
    pub expires_on: String,
}
