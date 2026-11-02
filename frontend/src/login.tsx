function Login() {
  return (
    <>
      <div className="flex w-88 flex-col items-center font-round">
        <p className="font-medium text-font-pressed text-center mb-8">
          Erstelle ein Konto oder melde dich an, um <br />
          fortzufahren
        </p>
        <div className="font-semibold">
          <button className="mb-4 bg-brand-bg border-brand-border border-2 h-14 w-88 rounded-full active:border-brand-border-pressed active:text-font-pressed">
            Login Code (Gast)
          </button>
          <button className="mb-4 bg-brand-accent h-14 w-88 rounded-full active:bg-brand-accent-pressed active:text-font-pressed">
            Email Adresse
          </button>
        </div>
      </div>
    </>
  );
}

export default Login;
