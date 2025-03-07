import { fail } from "@sveltejs/kit";

export const action ={
    default: async ({ request,fetch }) => {
        const data= await request.formData();
        const username = data.get('username');
        const password = data.get('password');

        try {
            const response = await fetch('/api/signup', {
                method: 'POST',
                Headers: { 'Contnet-Type': 'application/json' },
                body: JSON.stringify({ username, password })
            });

            if (!response.ok){
                const errorData = await response.json();
            }
        }

        const result =await.response.json();
        return {
            status: response.status,
            body: result
        };
}