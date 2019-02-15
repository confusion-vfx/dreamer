import QtQuick 2.3
import QtQuick.Controls 1.2

ApplicationWindow {
    id: mainWindow
       visible: true
       width: 600
       height: 400
       title: qsTr("Frameless Window!")

       flags: Qt.FramelessWindowHint

       property point startPoint: Qt.point(0, 0)
       property point offsetPoint: Qt.point(0, 0)
       property bool  isMoveWindow: false

       Rectangle {
               id: titleBarRectangle
               x: 0
               y: 0
               width: parent.width
               height: 75
               color: "#3b4852"

               MouseArea {
                   id: mouseMoveWindowArea
                   anchors.fill: parent
                   onPressed: {
                       cursorShape =
                               Qt.DragMoveCursor;
                       startPoint = Qt.point(mouseX, mouseY);
                       isMoveWindow = true;
                   }
                   onPositionChanged: {
                       mainWindow.offsetPoint = Qt.point(mouseX - mainWindow.startPoint.x,
                                                        mouseY - mainWindow.startPoint.y);
                       if(true == mainWindow.isMoveWindow)
                       {
                           mainWindow.x = mainWindow.x +mainWindow.offsetPoint.x;
                           mainWindow.y = mainWindow.y +mainWindow.offsetPoint.y;
                       }

                   }
                   onReleased: {
                       cursorShape =
                               Qt.ArrowCursor;
                       isMoveWindow = false;
                   }

               }
       }
}
