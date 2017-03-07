//
// Created by mistlight on 3/5/2017.
//

#include "AddRemoveVisitor.h"

AddRemoveVisitor::AddRemoveVisitor(Audiobook *audiobookModel) {
    this->audiobookModel = audiobookModel;
}

void AddRemoveVisitor::accept(QDir) {

}

void AddRemoveVisitor::accept(QFile) {

}

void AddRemoveVisitor::accept(QString directory) {

}
