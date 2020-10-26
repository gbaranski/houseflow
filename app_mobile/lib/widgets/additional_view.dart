import 'package:flutter/material.dart';
import 'package:houseflow/shared/constants.dart';

class AdditionalView extends StatelessWidget {
  final Widget body;
  AdditionalView({@required this.body});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(70),
        child: AppBar(
          backgroundColor: Colors.white,
          elevation: 0,
          flexibleSpace: Container(
            margin: const EdgeInsets.only(left: 15),
            alignment: Alignment.bottomLeft,
            child: IconButton(
              onPressed: () => Navigator.pop(context),
              tooltip: "Back",
              icon: Icon(
                Icons.arrow_back_ios,
                color: ACTION_ICON_COLOR,
              ),
            ),
          ),
        ),
      ),
      body: body,
    );
  }
}
