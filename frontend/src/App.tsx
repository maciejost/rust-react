import { useEffect, useState } from 'react'
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
				<section
					style={{
						position: 'absolute',
						top: '50%',
						left: '10%',
						transform: 'translateY(-50%)',
					}}
				>
					<p
						style={{
							padding: '1rem 2rem',
							borderRadius: '0.4rem',
							fontWeight: 'bold',
							fontSize: '1.25rem',
							background:
								validateEmail(email) === 'Ugyldig epostadresse'
									? '#EF959C'
									: '#94A89A',
						}}
					>
						Epost : {validateEmail(email)}
					</p>
					<p
						style={{
							padding: '1rem 2rem',
							borderRadius: '0.4rem',
							fontWeight: 'bold',
							fontSize: '1.25rem',
							background:
								validateAge(Number(age)) === 'Du er for ung'
									? '#EF959C'
									: '#94A89A',
						}}
					>
						Alder : {validateAge(Number(age))}
					</p>

					<p
						style={{
							padding: '1rem 2rem',
							borderRadius: '0.4rem',
							fontWeight: 'bold',
							fontSize: '1.25rem',
							background:
								validateName(name) === 'Ugyldig navn'
									? '#EF959C'
									: '#94A89A',
						}}
					>
						Navn : {validateName(name)}
					</p>
				</section>
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
