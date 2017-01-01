//
// Created by mistlight on 12/31/2016.
//

#include "SettingsForm.h"

SettingsForm::SettingsForm(Directory* directoryModel, QWidget *parent) :
  QWidget(parent), ui(new Ui::SettingsForm()) {
    ui->setupUi(this);
    ui->listView->setModel(directoryModel);
}

SettingsForm::~SettingsForm() {
    delete ui;
}
