import "./App.css";
import Login from "./login.tsx";
import Content from "./content.tsx";

function AuthButton(logged_in: boolean) {
  if (logged_in) {
    return (
      <button className="bg-brand-accent h-14 w-32 rounded-full font-medium font-round active:bg-brand-accent-pressed">
        Logout
      </button>
    );
  }
}

function ContentPage(logged_in: boolean) {
  if (!logged_in) {
    return (
      <div className="flex w-full flex-1 flex-col items-center justify-end">
        <Login />
      </div>
    );
  } else {
    return <Content />;
  }
}

function App() {
  const logged_in = false;

  return (
    <div className="flex h-screen flex-col font-google-sans">
      <section className="flex flex-row h-30 ml-8 mr-6 justify-between items-center">
        <img src="./src/assets/logo4.png" alt="logo" className="w-18" />

        <div className="">{AuthButton(logged_in)}</div>
      </section>

      <section className="flex w-full flex-1 flex-col items-center">
        {ContentPage(logged_in)}
      </section>
    </div>
  );
}

export default App;
