import React from 'react';
import { Field, Form, Formik } from 'formik';
import * as Yup from 'yup';
import { Link } from 'react-router-dom';

interface SignInValues {
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

type MayNull<T> = {
  [P in keyof T]: T[P] | null;
};

interface GoogleParams {
  // The Google client ID you registered with Google.
  client_id: string | null;
  // The URL to which you send the response to this request.
  redirect_uri: string | null;
  // A bookkeeping value that is passed back to Google unchanged in the redirect URI.
  state: string | null;
  // Optional: A space-delimited set of scope strings that specify the data Google is requesting authorization for.
  scope: string | null;
  // The type of value to return in the response. For the OAuth 2.0 authorization code flow, the response type is always code.
  response_type: string | null;
  // The Google Account language setting in RFC5646 format, used to localize your content in the user's preferred language.
  user_locale: string | null;
}

const getGoogleParams = (): GoogleParams => {
  const params = new URL(document.location.href).searchParams;

  return {
    client_id: params.get('client_id'),
    redirect_uri: params.get('redirect_uri'),
    state: params.get('state'),
    scope: params.get('scope'),
    response_type: params.get('response_type'),
    user_locale: params.get('user_locale'),
  };
};

const SignIn = () => {
  const initialValues: SignInValues = {
    email: '',
    password: '',
  };

  const googleParams = getGoogleParams();

  const onSubmit = (values: SignInValues) => {};

  return (
    <div>
      <h1>Sign in page</h1>
      <Formik
        initialValues={initialValues}
        validationSchema={SignInSchema}
        onSubmit={onSubmit}
      >
        {({ errors, touched }) => (
          <Form>
            <Field name="email" type="email" placeholder="Email" />
            {errors.email && touched.email ? <div>{errors.email}</div> : null}
            <br />
            <Field name="password" placeholder="Password" type="password" />
            {errors.password && touched.password ? (
              <div>{errors.password}</div>
            ) : null}
            <br />
            <button type="submit">Submit</button>
            <br />
            <Link to="/signup">Sign up</Link>
          </Form>
        )}
      </Formik>
    </div>
  );
};

export default SignIn;
