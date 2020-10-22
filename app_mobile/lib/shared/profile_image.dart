import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';

class ProfileImage extends StatelessWidget {
  final String imageUrl;
  final double size;

  ProfileImage({@required this.imageUrl, this.size = 24});

  @override
  Widget build(BuildContext context) {
    return imageUrl == null
        ? Icon(
            Icons.account_circle,
            color: Colors.black.withAlpha(180),
            size: size,
          )
        : CachedNetworkImage(
            imageUrl: imageUrl,
            imageBuilder: (context, imageProvider) => CircleAvatar(
              child: CircleAvatar(
                backgroundImage: imageProvider,
              ),
            ),
            placeholder: (context, url) => const CircularProgressIndicator(),
            errorWidget: (context, url, error) => Icon(Icons.error),
          );
  }
}
