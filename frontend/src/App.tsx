import './App.css'
import Login from './login.tsx'
import Content from './content.tsx'

function AuthButton(logged_in: boolean) {
  if (logged_in) {
    return (
      <button className="accentButton headerButton">Logout</button>
    );
  } 
}

function ContentPage(logged_in: boolean) {
  if(!logged_in) {
    return <Login/>
  } else {
    return <Content/>
  }
}

function App() {
  const logged_in = true;
  

  return (
    <>
      <section className="header">
        
        <img src="./src/assets/logo4.png" alt="logo" className="logo" />
        
        <div className="auth">
          {AuthButton(logged_in)}
        </div>
      </section>

      <section className="center content">
        {ContentPage(logged_in)}
      </section>

    </>
  )
}

export default App
