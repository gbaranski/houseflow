import 'package:flutter/material.dart';

class ProfileImage extends StatelessWidget {
  final String imageUrl;

  ProfileImage({this.imageUrl});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 200,
      width: 200,
      child: DecoratedBox(
        decoration: const BoxDecoration(
            color: Colors.black26,
            borderRadius: BorderRadius.all(Radius.circular(100))),
        child: imageUrl == null
            ? Icon(
                Icons.person_outline,
                color: Colors.white,
                size: 150,
              )
            : Image.network(imageUrl),
      ),
    );
  }
}
