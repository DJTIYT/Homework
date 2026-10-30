function Login() {
  return (
    <>
      <div className="flex w-88 flex-col items-center">
        <p className="font-medium text-font-pressed text-center mb-8">
          Erstelle ein Konto oder melde dich an, um <br />
          fortzufahren
        </p>
        <button className="mb-4 bg-brand-bg border-brand-border border-2 h-14 w-88 rounded-full font-semibold font-round active:border-brand-border-pressed active:text-font-pressed">
          Login Code (Gast)
        </button>
        <button className="mb-4 bg-brand-accent h-14 w-88 rounded-full font-semibold font-round active:bg-brand-accent-pressed active:text-font-pressed">
          Email Adresse
        </button>
      </div>
    </>
  );
}

export default Login;
