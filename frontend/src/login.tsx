function Login() {
    return (
        <>
            <div className="bottom login">
                <p className="bold loginText">Erstelle ein Konto oder melde dich an, um fortzufahren</p>
                <button className="authButton nonAccentButton">Login Code (Gast)</button>
                <button className="authButton accentButton">Email Adresse</button>
            </div>
        </>
    );
}

export default Login