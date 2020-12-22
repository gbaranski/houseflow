import React from 'react';
import { Field, Form, Formik } from 'formik';
import * as Yup from 'yup';

interface SignUpValues {
  email: string;
  password: string;
}

const SignInSchema = Yup.object().shape({
  email: Yup.string().email('Invalid email').required('Required'),
  password: Yup.string()
    .min(8, 'Password too short! Min 8 characters')
    .max(64, 'Password too long! Max 64 characters')
    .required('Required'),
});

const SignIn = () => {
  const initialValues: SignUpValues = {
    email: '',
    password: '',
  };

  return (
    <div>
      <h1>Sign in page</h1>
      <Formik
        initialValues={initialValues}
        validationSchema={SignInSchema}
        onSubmit={async (values) => {
          // same shape as initial values
          console.log({
            email: values.email,
            password: values.password,
          });
        }}
      >
        {({ errors, touched }) => (
          <Form>
            <Field name="email" type="email" />
            {errors.email && touched.email ? <div>{errors.email}</div> : null}
            <br />
            <Field name="password" />
            {errors.password && touched.password ? (
              <div>{errors.password}</div>
            ) : null}
            <br />
            <button type="submit">Submit</button>
          </Form>
        )}
      </Formik>
    </div>
  );
};

export default SignIn;
