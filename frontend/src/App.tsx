import { useState } from 'react'
import './App.css'
import {
	validate_email as validateEmail,
	validate_age as validateAge,
	validate_name as validateName,
} from 'wasm-hackathon'
import { TextInput } from '@fremtind/jkl-text-input-react'

function App() {
	const [email, setEmail] = useState('')
	const [age, setAge] = useState('')
	const [name, setName] = useState('')

	return (
		<>
			<main>
				<h1 className='jkl-title'>Rust + React</h1>
				<section>
					<TextInput
						label='Epostvalidering'
						placeholder='Skriv inn epost'
						errorLabel={
							validateEmail(email) === 'Ugyldig epostadresse' &&
							validateEmail(email)
						}
						value={email}
						onChange={e => setEmail(e.target.value)}
					/>
					<TextInput
						label='Alderssjekk'
						placeholder='Skriv inn alder'
						errorLabel={
							validateAge(Number(age)) === 'Du er for ung' &&
							validateAge(Number(age))
						}
						value={age}
						onChange={e => setAge(e.target.value)}
						type='number'
					/>
					<TextInput
						label='Navn'
						placeholder='Skriv inn navn'
						value={name}
						onChange={e => setName(e.target.value)}
						errorLabel={
							validateName(name) === 'Ugyldig navn' &&
							validateName(name)
						}
					/>
				</section>
			</main>
		</>
	)
}

export default App
