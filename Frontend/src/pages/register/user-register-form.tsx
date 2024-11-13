import { createForm } from '@tanstack/solid-form'
import { zodValidator } from '@tanstack/zod-form-adapter'
import { z } from 'zod'

function App() {
  const form = createForm(() => ({
    defaultValues: {
      fullName: '',
    },
    onSubmit: async ({ value }) => {
      // Do something with form data
      console.log(value)
    },
  }))

  return (
    <div>
      <h1>Simple Form Example</h1>
        <form
          onSubmit={(e) => {
            e.preventDefault()
            e.stopPropagation()
            form.handleSubmit()
          }}
        >
          <div>
            <form.Field
              name="fullName"
              children={(field) => (
                <input
                  name={field().name}
                  value={field().state.value}
                  onBlur={field().handleBlur}
                  onInput={(e) => field().handleChange(e.target.value)}
                />
              )}
            />
          </div>
          <button type="submit">Submit</button>
        </form>
    </div>
  )
}