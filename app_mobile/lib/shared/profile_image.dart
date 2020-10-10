import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';

class ProfileImage extends StatelessWidget {
  final String imageUrl;

  ProfileImage({@required this.imageUrl});

  @override
  Widget build(BuildContext context) {
    return imageUrl == null
        ? const Icon(Icons.person_outline)
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
