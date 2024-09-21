import React, {useState} from "react";
import { Link } from "react-router-dom";
import {Cog8ToothIcon, PlusIcon, TrashIcon} from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import { useStorePage } from "./hooks/useStorePage";
import { useTranslation } from "react-i18next";
import { Controller, useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { usePageCreateSchema } from "./schemas/page.create.schema";
import ErrorMessage from "../../components/ErrorMessage";
import slug from "slug";
import {
  CreatablePageType,
} from "../../types/page/CreatablePageType";
import {AvoRedPageDataTYpe, AvoRedPageFieldType} from "../../types/page/IPageModel";
import AvoredModal from "../../components/AvoredModal";
import {EditableFieldType} from "../../types/page/EditablePageType";
import _ from "lodash";

function PageCreate() {
  const [t] = useTranslation("global");
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [currentIndex, setCurrentIndex] = useState<number>(0);

  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
    setValue,
    getValues,
    trigger,
  } = useForm<CreatablePageType>({
    resolver: joiResolver(usePageCreateSchema(), { allowUnknown: true }),
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: "page_fields", //rename fields
  });

  const { mutate, error } = useStorePage();

  const addFieldOnClick = async (
      e: React.MouseEvent<HTMLElement>,
      max_index: number,
  ) => {
    e.preventDefault();
    e.stopPropagation();

    append({
      name: "",
      identifier: "",
      data_type: AvoRedPageDataTYpe.TEXT,
      field_type: AvoRedPageFieldType.TEXT,
      field_content: "",
    });
    await trigger("page_fields");

    setCurrentIndex(max_index);

    setIsOpen(true);
  };

  const deletePageFieldOnClick = (e: any, index: number) => {
    remove(index);
  };

  const submitHandler = async (data: CreatablePageType) => {
    mutate(data);
  };

  const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
    setValue("identifier", slug(e.currentTarget.value || ""));
  };

  const onPageFieldChange = async (
      index: number,
      field_type: AvoRedPageFieldType,
      data_type: AvoRedPageDataTYpe,
  ) => {
    setValue(`page_fields.${index}.field_type`, field_type);
    setValue(`page_fields.${index}.data_type`, data_type);
    await trigger(`page_fields.${index}`);
  };

  const renderField = (field: EditableFieldType, index: number) => {
    switch (field.field_type) {
      case "TEXT":
        return (
            <div className="mb-4">
              <InputField
                  label={t("field_content")}
                  placeholder={t("field_content")}
                  register={register(`page_fields.${index}.field_content`)}
              />
            </div>
        );
      default:
        return (
            <div className="mb-4">
              <label className="text-sm text-gray-600">
                {t!("field_content")}
              </label>
              <textarea
                  className="w-full rounded"
                  {...register(`page_fields.${index}.field_content`)}
              ></textarea>
            </div>
        );
    }
  };

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 pl-64 ">
        <div className="w-full">
          <div className="block rounded-lg p-6">
            <h1 className="text-xl font-semibold mb-4 text-gray-900">
              {t("page_information")}
            </h1>

            <form onSubmit={handleSubmit(submitHandler)}>
              {(_.size(fields) > 0) ?
                  <>
                    <AvoredModal
                        closeModal={() => setIsOpen(false)}
                        modal_header={`Page Field`}
                        modal_body={
                          <div className="block">
                            <div className="flex w-full">
                              <div className="flex-1 pr-3">
                                <div className="mb-3">
                                  current index: {currentIndex}
                                  <InputField
                                      placeholder={t("page_field_name")}
                                      label={t("page_field_name")}
                                      register={register(
                                          `page_fields.${currentIndex}.name`,
                                      )}
                                  />
                                </div>
                                <div className="mb-3">
                                  <InputField
                                      placeholder={t("page_field_identifier")}
                                      label={t("page_field_identifier")}
                                      register={register(
                                          `page_fields.${currentIndex}.identifier`,
                                      )}
                                  />
                                </div>
                              </div>
                              <div className="ml-auto">
                                <div className="w-64 border-l p-3 mr-auto">
                                  <div
                                      onClick={() =>
                                          onPageFieldChange(
                                              currentIndex,
                                              AvoRedPageFieldType.TEXT,
                                              AvoRedPageDataTYpe.TEXT,
                                          )
                                      }
                                      className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.TEXT ? "bg-primary-200" : "bg-gray-300"} 
                              ring-1 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                  >
                                    {t("text_field")}
                                  </div>
                                  <div
                                      onClick={() =>
                                          onPageFieldChange(
                                              currentIndex,
                                              AvoRedPageFieldType.TEXTAREA,
                                              AvoRedPageDataTYpe.TEXT,
                                          )
                                      }
                                      className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.TEXTAREA ? "bg-primary-200" : "bg-gray-300"}  
                            ring-1 mt-2 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                  >
                                    {t("textarea_field")}
                                  </div>
                                </div>
                              </div>
                            </div>
                            <hr className="mt-3" />
                            <div className="mt-3">
                              <button
                                  type="button"
                                  onClick={() => setIsOpen(false)}
                                  className="bg-primary-500 px-3 py-2 rounded text-white"
                              >
                                Create Page field
                              </button>
                            </div>
                          </div>
                        }
                        isOpen={isOpen}
                    ></AvoredModal>
                  </>
                 :
                    <></>
                }

              <div className="flex w-full">
                <div className="w-1/2">
                  <div className="mb-4">
                    <InputField
                      autoFocus={true}
                      label={t("name")}
                      placeholder={t("name")}
                      register={register("name")}
                      onKeyUp={(e) => onNameChange(e)}
                    />
                    <ErrorMessage
                      frontendErrors={errors}
                      backendErrors={error}
                      identifier="name"
                    />
                  </div>
                </div>
                <div className="w-1/2 ml-3">
                  <div className="mb-4">
                    <InputField
                      label={t("identifier")}
                      placeholder={t("identifier")}
                      register={register("identifier")}
                    />
                    <ErrorMessage
                      frontendErrors={errors}
                      backendErrors={error}
                      identifier="identifier"
                    />
                  </div>
                </div>
              </div>
              {/*}<!-- FIELD CARD -->*/}
              {fields.map((field, index) => {
                return (
                  <div
                    key={field.id}
                    className="relative hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                  >
                    <Controller
                      name={`page_fields.${index}`}
                      render={({field: page_field}) => {
                        return (
                            <>
                              <div
                                  onClick={(e) => deletePageFieldOnClick(e, index)}
                                  className="absolute right-0 top-0 mt-3 mr-5"
                              >
                                <TrashIcon className="w-4 h-4"/>
                              </div>
                              <div className="flex mt-3 w-full justify-center">
                                <div className="flex-1 p-3">
                                  <div className="p-3 bg-gray-200 rounded">
                                    <div className="flex text-sm w-full border-gray-300 border-b py-2">
                                      <div className="flex-1 overflow-hidden">
                                        <span>{page_field.value.name}</span>
                                        <span className="ml-1 text-xs text-gray-500">
                                        ({page_field.value.identifier})
                                      </span>
                                      </div>
                                      <div className="ml-auto">
                                        <button
                                            type="button"
                                            className="outline-none"
                                            onClick={() => setIsOpen(true)}
                                        >
                                          <Cog8ToothIcon className="w-5 h-5"/>
                                        </button>
                                      </div>
                                    </div>

                                    <InputField
                                        type="hidden"
                                        placeholder={t("data_type")}
                                        register={register(
                                            `page_fields.${index}.data_type`,
                                        )}
                                    />
                                    <InputField
                                        type="hidden"
                                        placeholder={t("field_type")}
                                        register={register(
                                            `page_fields.${index}.field_type`,
                                        )}
                                    />
                                    {renderField(page_field.value, index)}
                                  </div>
                                </div>
                              </div>
                            </>
                        );
                      }}
                      control={control}
                    />
                  </div>
                );
              })}

              <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                <button
                    type="button"
                    className="flex"
                    onClick={(e) => addFieldOnClick(e, fields.length)}
                >
                  <PlusIcon className="text-primary-500 h-6 w-6"/>
                  <span className="text-sm ml-1 text-primary-500">
                    {t("add_field")}
                  </span>
                </button>
              </div>

              <hr/>
              <div className="mt-5  flex items-center">
                <button
                    type="submit"
                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  {t("save")}
                </button>
                <Link
                    to={`/admin/page`}
                    className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                >
                  {t("cancel")}
                </Link>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PageCreate;
