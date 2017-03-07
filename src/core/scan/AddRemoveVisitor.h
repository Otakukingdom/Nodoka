//
// Created by mistlight on 3/5/2017.
//

#ifndef NODOKA_ADDREMOVEVISITOR_H
#define NODOKA_ADDREMOVEVISITOR_H


#include <src/model/Audiobook.h>
#include "AbstractElementScanner.h"

/**
 * AddRemoveVisitor is called whenever a directory is scanned, or re-scanned
 */
class AddRemoveVisitor : public AbstractElementScanner {
private:
    Audiobook* audiobookModel;

public:
    AddRemoveVisitor(Audiobook* audiobookModel);

    void accept(QDir);
    void accept(QFile);
    void accept(QString directory);

};


#endif //NODOKA_ADDREMOVEVISITOR_H
