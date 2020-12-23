import { Field, Form, Formik } from 'formik';
import React from 'react';
import { Link } from 'react-router-dom';
import { AUTH_URL } from '../config';
import * as Yup from 'yup';

interface SignUpValues {
  email: string;
  firstName: string;
  lastName: string;
  password: string;
}

const SignUpSchema = Yup.object().shape({
  email: Yup.string().email('Invalid email').required('Required'),
  firstName: Yup.string().required('Required'),
  lastName: Yup.string().required('Required'),
  password: Yup.string()
    .min(8, 'Password too short! Min 8 characters')
    .max(64, 'Password too long! Max 64 characters')
    .required('Required'),
});

const SignUp = () => {
  const initialValues: SignUpValues = {
    email: '',
    firstName: '',
    lastName: '',
    password: '',
  };

  const onSubmit = async (values: SignUpValues) => {
    const res = await fetch(`${AUTH_URL}/createUser`, {
      method: 'POST',
      body: JSON.stringify(values),
    });
    console.log({ status: res.status });
    console.log(await res.text());
  };

  return (
    <div>
      <h1>Sign up page</h1>
      <Formik
        initialValues={initialValues}
        validationSchema={SignUpSchema}
        onSubmit={onSubmit}
      >
        {({ errors, touched }) => (
          <Form>
            <Field name="firstName" type="text" placeholder="First name" />
            {errors.email && touched.email ? <div>{errors.email}</div> : null}
            <br />
            <Field name="lastName" type="text" placeholder="Last name" />
            {errors.email && touched.email ? <div>{errors.email}</div> : null}
            <br />
            <Field name="email" type="email" placeholder="Email" />
            {errors.email && touched.email ? <div>{errors.email}</div> : null}
            <br />
            <Field name="password" placeholder="Password" type="password" />
            {errors.password && touched.password ? (
              <div>{errors.password}</div>
            ) : null}
            <br />
            <button type="submit">Create account</button>
            <br />
            <Link to="/signin">Sign in</Link>
          </Form>
        )}
      </Formik>
    </div>
  );
};

export default SignUp;
