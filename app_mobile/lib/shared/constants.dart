import 'package:flutter/material.dart';

const textInputFillColor = Color.fromRGBO(240, 240, 240, 50);

const textInputRoundBorder = OutlineInputBorder(
  borderSide: BorderSide(style: BorderStyle.none),
  borderRadius: BorderRadius.all(Radius.elliptical(20, 20)),
);

const textInputRoundBorderRed = OutlineInputBorder(
  borderSide: BorderSide(color: Colors.red),
  borderRadius: BorderRadius.all(Radius.elliptical(20, 20)),
);

const textInputDecoration = InputDecoration(
  fillColor: textInputFillColor,
  filled: true,
  enabledBorder: textInputRoundBorder,
  focusedBorder: textInputRoundBorder,
  focusedErrorBorder: textInputRoundBorderRed,
  errorBorder: textInputRoundBorderRed,
);

const textInputTextStyle = TextStyle(
  fontSize: 14,
);

const Color LayoutBlueColor1 = Colors.blueAccent;

const Color NavigationUnselectedItemColor = Colors.blueGrey;
