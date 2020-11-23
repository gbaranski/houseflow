import 'package:flutter/material.dart';
import 'package:houseflow/shared/constants.dart';

class AdditionalView extends StatelessWidget {
  final Widget body;
  final FloatingActionButton floatingActionButton;
  AdditionalView({@required this.body, this.floatingActionButton});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      floatingActionButton: floatingActionButton,
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            backgroundColor: Colors.white,
            elevation: 0,
            expandedHeight: 100,
            leading: Padding(
              padding: const EdgeInsets.only(top: 25, left: 15),
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
          SliverToBoxAdapter(child: body)
        ],
        physics:
            AlwaysScrollableScrollPhysics().applyTo(BouncingScrollPhysics()),
      ),
    );
  }
}
