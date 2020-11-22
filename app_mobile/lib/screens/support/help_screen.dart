import 'package:houseflow/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/screens/additional_view/index.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';
import 'package:url_launcher/url_launcher.dart';

class HelpScreen extends StatelessWidget {
  static void launchUrl(BuildContext context, String url) async {
    if (await canLaunch(url)) {
      await launch(url);
    } else {
      const SnackBar snackBar =
          SnackBar(content: Text("Something went wrong!"));
      Scaffold.of(context).showSnackBar(snackBar);
    }
  }

  @override
  Widget build(BuildContext context) {
    return AdditionalView(
      body: Container(
        margin: const EdgeInsets.symmetric(vertical: 20, horizontal: 20),
        alignment: Alignment.topCenter,
        child: Column(children: [
          const Text(
            "Looks like you've got a problem\nPlease contact us via methods below",
            style: const TextStyle(fontSize: 18),
          ),
          const SizedBox(
            height: 15,
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              IconButton(
                icon: const Icon(
                  Icons.email_outlined,
                  size: 48,
                  color: Colors.black87,
                ),
                onPressed: () => launchUrl(context, SUPPORT_EMAIL_URL_ISSUE),
                tooltip: SUPPORT_EMAIL,
              ),
              const SizedBox(
                width: 20,
              ),
              IconButton(
                icon: const Icon(
                  MdiIcons.github,
                  size: 48,
                  color: Colors.black87,
                ),
                tooltip: GITHUB_URL,
                onPressed: () => launchUrl(context, GITHUB_URL_ISSUES),
              ),
            ],
          )
        ]),
      ),
    );
  }
}
